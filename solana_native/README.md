

# 命名规范

由 next_account_info 所得的, 若是程序则 _program ,非程序则 _account 结尾

Mint Account 命名 mint_mintname 开头 , Token Account 命名 token_mintname 开头, Owner Account 命名 owner_mintname
例如 mint_rmb, token_rmb , owner_rmb

todo

# 当前程序问题
有许多优化没做,懒得去做,有时间再做吧

# Note
seeds 长度是 16个
seed 长度是32字节

可能 调用某些程序的指令时, AccountList 必须包含那个程序的Account
