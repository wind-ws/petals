use std::{
    collections::{HashMap, HashSet},
    iter::Map,
    str::FromStr,
};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

/// PDA(init)
///
/// 存储上限是 10MB(10485760 Byte)
/// id(1) + next(32) + (list(32)+_a(4))*element number
///
/// 元素上限是 291270 个元素
///
/// list的容量增长与_a同步 (即使_a可能用不到)(有优化空间)
///
/// 优化: 取消使用Vec, 自己管理和分配数组大小, 管理总长度
///
/// plan: 移除 [id,_a,next](虽然可以提高性能,但是搞的麻烦,一个练手项目,没啥必要)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RaiseFundList {
    // /// 表示List的id, 当前id=0,next的id=1,以此下推
    // pub id: u8,
    /// <raise_fund pda>
    /// Pubkey被分配后 索引不变,直到被删除
    ///
    /// 初始空间分配为 4 ,每次增长为*2
    /// 增长到 262144 后 下次增长为291270
    /// 291270 后,元素添加进入 next
    list: Vec<Pubkey>,
    // /// 记录 list 中被移除的 index ,而非调整vec大小(重排序,重分配)
    // /// 也确保被分配的索引不变
    // _a: Vec<u32>,

    // /// 若空间不够,需要继续扩展
    // pub next: Option<Pubkey>,
}

impl RaiseFundList {
    // pub fn new(id: u8) -> Self {
    //     Self {
    //         id,
    //         list: Vec::with_capacity(4),
    //         _a: Vec::with_capacity(4),
    //         next: None,
    //     }
    // }
    // /// true: 增长成功 false:容量达到上限
    // ///
    // /// note: 需要提前为存储的账户申请空间
    // pub fn grow_capacity(&mut self) -> bool {
    //     let capacity = self.list.capacity();
    //     if capacity == 262144 {
    //         self.list.reserve_exact(291270 - capacity);
    //         self._a.reserve_exact(291270 - capacity);
    //         true
    //     } else if capacity == 291270 {
    //         false
    //     } else {
    //         self.list.reserve_exact(capacity);
    //         self._a.reserve_exact(capacity);
    //         true
    //     }
    // }
    // /// return : 存储的下标
    // pub fn push(&mut self, v: Pubkey) -> u32 {
    //     if self._a.len() != 0 {
    //         let index = self._a.pop().unwrap();
    //         self.list[index as usize] = v;
    //         index
    //     } else {
    //         0
    //     }
    // }
    // pub fn remove(&mut self, id: u8, index: u32) {}

    pub fn space() -> usize {
        todo!()
    }
    pub fn new() -> Self {
        let list = Vec::with_capacity(4);
        Self { list }
    }
    pub fn seed(id: u8) -> [u8; 16] {
        const SEED: &[u8; 15] = b"raise_fund_list";
        let mut seed = [0u8; 16];
        seed[..15].copy_from_slice(SEED);
        seed[15] = id;
        seed
    }
    pub fn pda_id(program_id: &Pubkey, id: u8) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&RaiseFundList::seed(id)], program_id)
    }
    /// id = 0
    /// 仍保留id,未来方便扩展
    pub fn pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[&RaiseFundList::seed(0)], program_id)
    }
}

/// PDA
/// 募捐用户pda的实际存储类型
pub struct RaiseFund {
    info: RaiseFundInfo,
    /// 需要的捐款数量
    require_amount: u64,
    /// 以捐款的数量
    amount: u64,
    /// 以被取走的捐款数量
    take_amount: u64,

    /// 收款方 token_rmb_pubkey
    payee: Pubkey,

    /// ture:是运行状态(true:可被捐款)
    is_run: bool,
}
impl RaiseFund {
    /// seed = "raise_fund"+[RealName]+[IDNumber]+[payee PubKey]
    pub fn pda(
        program_id: &Pubkey,
        real_name: &String,
        id: &String,
        payee: &Pubkey,
    ) {
        Pubkey::find_program_address(
            &[
                b"raise_fund",
                real_name.as_bytes(),
                id.as_bytes(),
                &payee.to_bytes(),
            ],
            program_id,
        );
    }
    pub fn new(
        info: RaiseFundInfo,
        require_amount: u64,
        payee: Pubkey,
    ) -> Self {
        Self {
            info,
            require_amount,
            amount: 0,
            take_amount: 0,
            payee,
            // payer:HashSet::new(1),
            is_run: true,
        }
    }
}

/// PDA (发布募捐必须具有这个PDA,捐款者可有可无)
///
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserInfo {
    /// 初始化后不可修改
    base_info: BaseInfo,
    /// <捐助方(token_rmb),数量>
    payer: HashMap<Pubkey, u64>,
}
impl UserInfo {
    // pub const INIT_SPACE: u64 = 99;
    /// seed = "user_info"+token_rmb
    pub fn pda(program_id: &Pubkey, token_rmb: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"user_info", &token_rmb.to_bytes()],
            program_id,
        )
    }
    pub fn new(base_info: BaseInfo) -> Self {
        Self {
            base_info,
            payer: HashMap::new(),
        }
    }
    /// 需要的空间大小 (单位字节)
    pub fn space(&self) -> u64 {
        let space = 0;
        todo!()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BaseInfo {
    /// 真实姓名(需要被募捐的)
    name: String,
    /// 手机号码
    phone: PhoneNumber,
    /// 身份证号码,全局唯一
    id: String,
}
impl BaseInfo {
    pub fn new(name: String, phone: PhoneNumber, id: String) -> Self {
        Self { name, phone, id }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn phone(&self) -> &PhoneNumber {
        &self.phone
    }
    pub fn id(&self) -> &String {
        &self.id
    }
}

/// 国际区号
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub enum AreaCode {
    /// +86
    #[default]
    China,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PhoneNumber {
    pub area_code: AreaCode,
    pub number: u64,
}
impl PhoneNumber {
    pub fn new(number: u64) -> Self {
        Self {
            area_code: AreaCode::China,
            number,
        }
    }
}

/// 用户信息
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RaiseFundInfo {
    /// 标题
    title: String,
    /// 简介
    brief: String,
    /// 图片URL来源, 多张
    picture_url: Option<String>,
    /// URL来源的多张图片的 hash
    picture_hash: Option<()>,
}
