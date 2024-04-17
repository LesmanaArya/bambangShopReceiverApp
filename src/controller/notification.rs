use rocket::serde::json::Json;

use bambangshop_reciever::Result;
use crate::model::notification::Notification;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;