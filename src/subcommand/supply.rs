use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub supply: u64,
  pub first: u64,
  pub last: u64,
  pub last_mined_in_block: u32,
}

pub(crate) fn run() -> SubcommandResult {
  let mut last = 0;
/*
  loop {
    if Height(last + 1).subsidy() == 0 {
      break;
    }
    last += 1;
  }
*/
  Ok(Some(Box::new(Output {
    supply: 0,//ppctodo get supply from rpc Sat::SUPPLY,
    first: 0,
    last: 0, //Sat::SUPPLY - 1,
    last_mined_in_block: last,
  })))
}
