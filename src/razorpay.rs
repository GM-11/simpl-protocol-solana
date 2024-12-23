use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    name: String,
    email: String,
    contact: String,
    #[serde(rename = "type")]
    contact_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccount {
    contact_id: String,
    account_type: String,
    bank_account: BankAccount,
}
#[derive(Serialize, Deserialize, Debug)]
struct BankAccount {
    name: String,
    account_number: String,
    ifsc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePayout {
    account_number: String,
    fund_account_id: String,
    amount: u64,
    currency: String,
    mode: String,
    purpose: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayoutResponse {
    status: String,
}

async fn create_contact(contact: Contact) -> Result<String, String> {
    let url = "https://api.razorpay.com/v1/contacts";
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&contact)
        .header("Content-Type", "application/json")
        .basic_auth("username", Some("()"))
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let response: Response = res.json().await.unwrap();
                Ok(response.id)
            } else {
                println!("Error: {:?}", res.status());
                Err(res.status().as_str().to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
async fn create_fund_account(fund_account: FundAccount) -> Result<String, String> {
    let url = "https://api.razorpay.com/v1/fund_accounts";
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&fund_account)
        .header("Content-Type", "application/json")
        .basic_auth("username", Some("()"))
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let response: Response = res.json().await.unwrap();
                Ok(response.id)
            } else {
                println!("Error: {:?}", res.status());
                Err(res.status().as_str().to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

async fn create_payout(payout: CreatePayout) -> Result<String, String> {
    let url = "https://api.razorpay.com/v1/payouts";
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&payout)
        .header("Content-Type", "application/json")
        .header("X-Payout-Idempotency", "")
        .basic_auth("username", Some("()"))
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let response: Response = res.json().await.unwrap();
                Ok(response.id)
            } else {
                println!("Error: {:?}", res.status());
                Err(res.status().as_str().to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
async fn approve_payout(payout_id: String) -> Result<String, String> {
    let url = format!("https://api.razorpay.com/v1/payouts/{}/approve", payout_id);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer"))
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let response: Response = res.json().await.unwrap();
                Ok(response.id)
            } else {
                println!("Error: {:?}", res.status());
                Err(res.status().as_str().to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn pay(
    name: String,
    email: String,
    contact: String,
    account_number: String,
    ifsc: String,
    amount: u64,
) -> Result<String, String> {
    let contact = Contact {
        name: name.clone(),
        email,
        contact,
        contact_type: "customer".to_string(),
    };

    let contact_id = create_contact(contact).await;
    if contact_id.is_err() {
        return Err(contact_id.err().unwrap());
    }

    let fund = create_fund_account(FundAccount {
        contact_id: contact_id.unwrap(),
        account_type: "bank_account".to_string(),
        bank_account: BankAccount {
            name: name.clone(),
            account_number: account_number.clone(),
            ifsc,
        },
    })
    .await;

    if fund.is_err() {
        return Err(fund.err().unwrap());
    }

    let payout = create_payout(CreatePayout {
        account_number,
        fund_account_id: fund.unwrap(),
        amount,
        currency: "INR".to_string(),
        mode: "IMPS".to_string(),
        purpose: "payout".to_string(),
    })
    .await;

    if payout.is_err() {
        return Err(payout.err().unwrap());
    }

    let payout_id = payout.unwrap();
    let approve = approve_payout(payout_id).await;
    if approve.is_err() {
        return Err(approve.err().unwrap());
    }

    Ok(approve.unwrap().to_string())
}
