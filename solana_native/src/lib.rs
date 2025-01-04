#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
mod processor;
mod instructions;
mod state;
mod error;
mod instruction;
mod accounts;

#[cfg(test)]
mod tests;




