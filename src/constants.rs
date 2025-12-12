// This module contains constants used throughout the Monnify SDK.
// This includes constants for API endpoints, authentication, and other configuration settings.

pub const MONNIFY_API_BASE_URL: &str = "https://sandbox.monnify.com";
pub const MONNIFY_AUTHENTICATION_ENDPOINT: &str = "/api/v1/auth/login";

// Transactions endpoints
pub const MONNIFY_INIT_TRANSACTION_ENDPOINT: &str =
    "/api/v1/merchant/transactions/init-transaction";
pub const MONNIFY_INIT_BANK_TRANSFER_ENDPOINT: &str = "/api/v1/merchant/bank-transfer/init-payment";
pub const MONNIFY_INIT_CARD_CHARGE_ENDPOINT: &str = "/api/v1/merchant/cards/charge";
pub const MONNIFY_INIT_CARD_OTP_AUTHORIZE_ENDPOINT: &str = "/api/v1/merchant/cards/otp/authorize";
pub const MONNIFY_INIT_CARD_SECURE_3D_AUTHORIZE_ENDPOINT: &str =
    "/api/v1/sdk/cards/secure-3d/authorize";
pub const MONNIFY_SEARCH_TRANSACTIONS_ENDPOINT: &str = "/api/v1/transactions/search";
pub const MONNIFY_GET_TRANSACTION_ENDPOINT: &str = "/api/v2/transactions/{transactionReference}";
pub const MONNIFY_QUERY_TRANSACTIONS_ENDPOINT: &str = "/api/v2/merchant/transactions/query";
