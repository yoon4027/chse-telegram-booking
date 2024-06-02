use axum::{extract::Path, response::Response, routing::get, Router};

use crate::model::{Seat, UserTicketOrder};

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    Database,
};

pub async fn start_verifier_web_server(db: Database) {
    let app = Router::new().route(
        "/verify/:order_id",
        get(move |path| verify(path, db.clone())),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3420")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn verify(Path(order_id): Path<String>, db: Database) -> Response<String> {
    let order_id = match ObjectId::parse_str(&order_id).ok() {
        Some(n) => n,
        None => {
            return Response::builder()
                .status(400)
                .header("Content-Type", "text/html")
                .body(error_message("Invalid order ID!"))
                .unwrap();
        }
    };

    let ticket_order = match db
        .collection::<UserTicketOrder>("approved_orders")
        .find_one(doc! { "_id": Bson::ObjectId(order_id) }, None)
        .await
    {
        Ok(n) => n,
        Err(e) => {
            return Response::builder()
                .status(500)
                .header("Content-Type", "text/html")
                .body(error_message(&format!("DB ERROR: {}", e)))
                .unwrap();
        }
    };

    let ticket_order = match ticket_order {
        Some(n) => n,
        None => {
            return Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .body(invalid_html())
                .unwrap();
        }
    };

    let seats_night_1 = ticket_order
        .seats
        .clone()
        .into_iter()
        .filter(|x| x.seat_number.starts_with("night1"))
        .collect::<Vec<_>>();

    let seats_night_2 = ticket_order
        .seats
        .clone()
        .into_iter()
        .filter(|x| x.seat_number.starts_with("night2"))
        .collect::<Vec<_>>();

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(success_html(
            &ticket_order.user_info.name,
            &ticket_order.user_info.phone_number,
            &seats_night_1,
            &seats_night_2,
        ))
        .unwrap()
}

fn success_html(
    name: &str,
    phone_number: &str,
    seats_night1: &[Seat],
    seats_night2: &[Seat],
) -> String {
    let success_html = include_str!("../../html/success.html");

    let success_html = success_html.replace("$name$", name);
    let success_html = success_html.replace("$phone_number$", phone_number);

    let success_html = success_html.replace(
        "$seats_night1$",
        &seats_night1
            .iter()
            .map(|x| x.seat_number.replace("night1-", "").to_string())
            .collect::<Vec<_>>()
            .join("<br />"),
    );

    let success_html = success_html.replace(
        "$seats_night2$",
        &seats_night2
            .iter()
            .map(|x| x.seat_number.replace("night2-", "").to_string())
            .collect::<Vec<_>>()
            .join("<br />"),
    );

    success_html
}

fn invalid_html() -> String {
    include_str!("../../html/invalid.html").to_string()
}

fn error_message(error: &str) -> String {
    let error_html = include_str!("../../html/error.html");
    let error_html = error_html.replace("$error_message$", error);
    error_html
}
