

// create methods for login/ logout / create
// this gets used for the signing etc to the surrealdb so the financial data is double protected because the user hast to authenticate when starting the tool and then we manage the jwt in the backend for him

pub mod login;
pub mod logout;
pub mod signup;


