use crate::account::{AccountError, create, get_by_username, list};

mod account;

fn main() {
    let new_user = create("abc_cho","abc choco");
    println!("new user({}) is created.", new_user.id);
    // ex1. 아래의 println 매크로를 요구사항에 맞춰 동작하게끔 account/mod.rs의 코드를 수정하시오
    /* 출력 결과는 다음과 같아야 한다
    Account {
        id: ...,
        username: ...,
        display_name: ...,
    }
    */
    // println!("{}", new_user);

    // ex2. account::create(...)는 내부적으로 literal 생성 스타일을 사용 중인데 newType 패턴을 사용하도록 리펙토링하시오

    println!();
    // ex3. account::list(...) 가 10 개의 account를 리턴하도록 해당 함수를 완성하시오
    let users = list().unwrap();
    println!("there are {} user(s).", users.len());
    println!("{:#?}", users);


    // ex4.
    // - 알수 없는 에러
    // - 중복 username,
    // - 중복 disaply_name
    // - not found
    // 등을 가지고 있는 account::AccountError Enum을 완성하고
    // 다음의 get_by_username 호출시에 AccountError중 하나를 발생시키도록 get_by_username을 완성하시오
    println!();

    // 아래의 주석을 제거하고 동작 하도록 만드시오

    // let target_user = "unknown";
    // let search_res = get_by_username(target_user);
    //
    // if search_res.is_err() && search_res.unwrap_err() == AccountError::??? {
    //     println!("Failed to search {} user for some unknown reason", target_user);
    // }
}
