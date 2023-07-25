#[macro_export]
macro_rules! unwrap_or_return_default {
  ($res:expr, $default:expr) => {
      match $res {
          Some(req_body) => req_body,
          None => return $default,
      }
  };
  
  ($res:expr, $default:expr, $warning_message:expr) => {
      match $res {
          Some(req_body) => req_body,
          None => {
              println!("\n | {} | \n", stringify!($warning_message));
              return $default;
          }
      }
  };
}