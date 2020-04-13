//! Koleksi query yang digunakan untuk operasi pada rest API Pandemia
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{
        error::{param_error, Error},
        ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest,
    },
    auth,
<<<<<<< HEAD
    dao::{Logs, RecordDao, SubReportDao, VillageDao},
=======
    dao::{Logs, RecordDao, SubReportDao},
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
    error::{self, ErrorCode},
    eventstream::{self, Event::NewRecordUpdate},
    models,
    prelude::*,
    sub_report_dao,
    types::{HealthyKind, LocKind, SubReportStatus},
    ID,
};

#[derive(Deserialize, Validate)]
pub struct RecordUpdate {
    #[validate(range(min = 1, max = 9999999999))]
    pub id: i64,
    #[validate(length(min = 2, max = 1000))]
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRecords {
    records: Vec<RecordUpdate>,
}

#[derive(Serialize)]
pub struct InfoLocation {
    pub name: String,
    pub latest_record: models::Record,
    pub history: Vec<models::Record>,
}

#[derive(Serialize, Deserialize)]
pub struct SubReportQuery {
    pub offset: i64,
    pub limit: i64,
    pub query: Option<String>,
    pub status: i32,
}

#[derive(Deserialize, Validate)]
pub struct AddRecord {
    #[validate(length(min = 2, max = 1000))]
    pub loc: String,
    #[validate(range(min = 0, max = 20))]
    pub loc_kind: i16,
    #[validate(length(min = 2, max = 1000))]
    pub loc_scope: String,
    #[validate(range(min = 0))]
    pub total_cases: i32,
    #[validate(range(min = 0))]
    pub total_deaths: i32,
    #[validate(range(min = 0))]
    pub total_recovered: i32,
    #[validate(range(min = 0))]
    pub active_cases: i32,
    #[validate(range(min = 0))]
    pub critical_cases: i32,
}

#[derive(Deserialize, Validate)]
pub struct AddSubReport {
    #[validate(length(min = 1, max = 50))]
    pub full_name: String,
    pub age: i32,
    #[validate(length(min = 1, max = 80))]
    pub residence_address: String,
    #[validate(length(min = 1, max = 50))]
    pub gender: String,
    #[validate(length(min = 1, max = 70))]
<<<<<<< HEAD
    pub coming_from: String,
=======
    pub arrival_address: String,
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
    pub arrival_date: NaiveDate,
    #[validate(length(min = 1, max = 50))]
    pub desc: String,
    pub status: i32,
    pub complaint: Option<Vec<String>>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateSubReport {
    pub id: ID,
    #[validate(length(min = 1, max = 50))]
    pub full_name: String,
    pub age: i32,
    #[validate(length(min = 1, max = 80))]
    pub residence_address: String,
    #[validate(length(min = 1, max = 50))]
    pub gender: String,
    #[validate(length(min = 1, max = 70))]
<<<<<<< HEAD
    pub coming_from: String,
=======
    pub arrival_address: String,
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
    pub arrival_date: NaiveDate,
    #[validate(length(min = 1, max = 50))]
    pub desc: String,
    pub status: i32,
    pub complaint: Option<Vec<String>>,
}

/// Holder untuk implementasi API endpoint publik untuk Pandemia.
pub struct PublicApi;

#[api_group("Pandemia", "public", base = "/pandemia/v1")]
impl PublicApi {
    /// Add record.
    #[api_endpoint(path = "/add_record", auth = "required", mutable, accessor = "admin")]
    pub fn add_record(query: AddRecord) -> ApiResult<models::Record> {
        query.validate()?;
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let record = dao.create(
            &query.loc,
            query.loc_kind.into(),
            query.total_cases,
            query.total_deaths,
            query.total_recovered,
            query.active_cases,
            query.critical_cases,
            &vec![&format!("loc_scope:{}", query.loc_scope)],
            false,
        )?;

        eventstream::emit(NewRecordUpdate(None, record.clone()));

        Logs::new(&conn).write(
            &format!(
                "{} add new record for {}. total_cases: {}, total_deaths: {}, total_recovered: {}",
                current_admin.name, query.loc, query.total_cases, query.total_deaths, query.total_recovered
            ),
            current_admin.id,
        );

        Ok(ApiResult::success(record))
    }

    /// Add Sub Report.
    #[api_endpoint(path = "/sub_report/add", auth = "required", accessor = "user", mutable)]
    pub fn add_sub_report(query: AddSubReport) -> ApiResult<models::SubReport> {
        query.validate()?;
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        if !current_user.is_satgas() {
            param_error("Anda tidak dapat menambahkan data")?;
        }

        let mut healthy: HealthyKind = HealthyKind::Health;
        let mut meta: Vec<String> = Vec::new();
        if let Some(complaint) = &query.complaint.as_ref() {
            if complaint.len() > 0 {
                healthy = HealthyKind::Sick;
                meta.push(format!("gejala={}", complaint.join(",")))
            }
        }
        let status = match query.status {
            0 => SubReportStatus::ODP,
            1 => SubReportStatus::PDP,
            2 => SubReportStatus::Positive,
            3 => SubReportStatus::Recovered,
            _ => param_error("Status tidak valid")?,
        };
        let sub_report = dao.create(
            current_user.id,
            &current_user.full_name,
            &query.full_name,
            query.age,
            &query.residence_address,
            &query.gender,
<<<<<<< HEAD
            &query.coming_from,
=======
            &query.arrival_address,
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
            query.arrival_date,
            healthy as i32,
            &query.desc,
            status as i32,
            &meta.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
        )?;
        Ok(ApiResult::success(sub_report))
    }

    /// Update Sub Report.
    #[api_endpoint(path = "/sub_report/update", auth = "required", accessor = "user", mutable)]
<<<<<<< HEAD
    pub fn update_sub_report(query: UpdateSubReport) -> ApiResult<models::SubReport> {
=======
    pub fn update_sub_report(query: UpdateSubReport) -> ApiResult<()> {
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
        query.validate()?;
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        if !current_user.is_satgas() {
            param_error("Anda tidak dapat menambahkan data")?;
        }

        let mut healthy: HealthyKind = HealthyKind::Health;
        let mut meta: Vec<String> = Vec::new();
        if let Some(complaint) = &query.complaint.as_ref() {
            if complaint.len() > 0 {
                healthy = HealthyKind::Sick;
                meta.push(format!("gejala={}", complaint.join(",")))
            }
        }
        let status = match query.status {
            0 => SubReportStatus::ODP,
            1 => SubReportStatus::PDP,
            2 => SubReportStatus::Positive,
            3 => SubReportStatus::Recovered,
            _ => param_error("Status tidak valid")?,
        };
        let sub_report = dao.update(
            query.id,
            sub_report_dao::UpdateSubReport {
                full_name: &query.full_name,
                age: query.age,
                residence_address: &query.residence_address,
                gender: &query.gender,
<<<<<<< HEAD
                coming_from: &query.coming_from,
=======
                arrival_address: &query.arrival_address,
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
                arrival_date: query.arrival_date,
                healthy: healthy as i32,
                desc: &query.desc,
                status: status as i32,
                meta: &meta.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
            },
        )?;
<<<<<<< HEAD
        Ok(ApiResult::success(sub_report))
=======
        Ok(ApiResult::success(()))
>>>>>>> 4bf8d35... [PAND-23] Buat multiline input alamat pada screen tambah data
    }

    /// Search for sub_report
    #[api_endpoint(path = "/sub_report/search", auth = "required", accessor = "user")]
    pub fn search_sub_reports(query: SubReportQuery) -> ApiResult<EntriesResult<models::SubReport>> {
        let conn = state.db();
        let dao = SubReportDao::new(&conn);

        let status = match query.status {
            0 => SubReportStatus::ODP,
            1 => SubReportStatus::PDP,
            2 => SubReportStatus::Positive,
            3 => SubReportStatus::Recovered,
            _ => param_error("Status tidak valid")?,
        };

        let result = dao.search(
            current_user.id,
            status as i32,
            &query.query.unwrap_or("".to_string()),
            query.offset,
            query.limit,
        )?;

        Ok(ApiResult::success(EntriesResult {
            entries: result.entries,
            count: result.count,
        }))
    }

    /// Get location stats data (single mode).
    #[api_endpoint(path = "/info_location", auth = "none")]
    pub fn get_info_location(query: LocationQuery) -> ApiResult<Option<models::Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let locs: Vec<&str> = vec![query.loc.as_str()];
        let mut rec = dao.get_latest_records(locs, 0, 3)?;

        if rec.first().is_some() {
            Ok(ApiResult::success(Some(rec.swap_remove(0))))
        } else {
            Ok(ApiResult::success(None))
        }
    }

    /// Get per location stats data, use comma for multiple locations.
    #[api_endpoint(path = "/info_locations", auth = "none")]
    pub fn get_info_locations(query: LocationQuery) -> ApiResult<Vec<InfoLocation>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let locs: Vec<&str> = query.loc.split(',').collect();

        let records = dao.get_latest_records(locs, 0, 10)?;

        let mut result = vec![];

        for rec in records {
            let mut history: Vec<models::Record> = vec![];

            if query.with_history == Some(true) {
                history = dao.get_record_history(&rec.loc, 0, 30)?;
            }

            result.push(InfoLocation {
                name: rec.loc.to_owned(),
                latest_record: rec,
                history,
            });
        }

        Ok(ApiResult::success(result))
    }

    /// Get latest data record search/query by location.
    #[api_endpoint(path = "/search_records", auth = "required", accessor = "admin")]
    pub fn search_records(query: QueryEntries) -> ApiResult<EntriesResult<Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let result = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: result.count,
            entries: result.entries.into_iter().map(|a| a.to_api_type(&conn)).collect(),
        }))
    }

    /// Update multiple records at once.
    #[api_endpoint(path = "/update_records", auth = "required", mutable, accessor = "admin")]
    pub fn update_records(query: UpdateRecords) -> ApiResult<()> {
        use crate::schema::records::{self, dsl};
        query.validate()?;

        let conn = state.db();

        let locs = query
            .records
            .iter()
            .map(|a| a.loc.to_owned())
            .collect::<Vec<String>>();

        conn.build_transaction()
            .read_write()
            .run::<_, error::Error, _>(|| {
                let dao = RecordDao::new(&conn);

                for record in query.records {
                    let old_record = dao.get_latest_records(vec![record.loc.as_ref()], 0, 1)?.pop();

                    let new_record = dao.create(
                        &record.loc,
                        record.loc_kind.into(),
                        record.total_cases,
                        record.total_deaths,
                        record.total_recovered,
                        record.active_cases,
                        record.critical_cases,
                        &record.meta.iter().map(|a| a.as_str()).collect(),
                        true,
                    )?;

                    if let Some(old_record) = old_record {
                        let diff = new_record.diff(&old_record);

                        if diff.new_cases > 0
                            || diff.new_deaths > 0
                            || diff.new_recovered > 0
                            || diff.new_critical > 0
                        {
                            eventstream::emit(NewRecordUpdate(Some(old_record.clone()), new_record.clone()));
                        }
                    }

                    debug!("updating record {}...", record.id);
                }

                Ok(())
            })?;

        Logs::new(&conn).write(
            &format!(
                "{} update record for {} records: {}",
                current_admin.name,
                locs.len(),
                locs.join(", ")
            ),
            current_admin.id,
        );

        Ok(ApiResult::success(()))
    }

    /// Delete record by id
    #[api_endpoint(path = "/delete_record", auth = "required", mutable, accessor = "admin")]
    pub fn delete_record(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let rec = dao.get_by_id(query.id)?;
        dao.delete_by_id(rec.id)?;

        Logs::new(&conn).write(
            &format!("{} delete record for {}", current_admin.name, rec.loc),
            current_admin.id,
        );

        Ok(ApiResult::success(()))
    }

    /// Search for journal_logs
    #[api_endpoint(path = "/journal/search", auth = "required", accessor = "admin")]
    pub fn search_journal_logs(query: QueryEntries) -> ApiResult<EntriesResult<models::Log>> {
        let conn = state.db();
        let dao = Logs::new(&conn);

        let rv = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: rv.count,
            entries: rv.entries,
        }))
    }

    /// Add village.
    #[api_endpoint(path = "/village/add", auth = "required", mutable, accessor = "admin")]
    pub fn add_village(query: AddVillage) -> ApiResult<models::Village> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let village = dao.create(
            &query.name,
            &query.sub_district,
            &query.city,
            &query.province,
            query.latitude.parse::<f64>()?,
            query.longitude.parse::<f64>()?,
            &vec![],
        )?;
        Ok(ApiResult::success(village))
    }

    /// Search for villages
    #[api_endpoint(path = "/village/search", auth = "optional")]
    pub fn search_villages(query: QueryEntries) -> ApiResult<EntriesResult<models::Village>> {
        query.validate()?;
        let conn = state.db();
        let dao = VillageDao::new(&conn);

        let sresult = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        // let entries = sresult.entries.into_iter().map(|p| p.into()).collect();

        let count = sresult.count;
        Ok(ApiResult::success(EntriesResult {
            count,
            entries: sresult.entries,
        }))
    }

    /// Delete village.
    #[api_endpoint(path = "/villages/delete", auth = "required", mutable, accessor = "admin")]
    pub fn delete_village(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();

        let dao = VillageDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }
}

#[derive(Deserialize, Validate)]
pub struct AddVillage {
    #[validate(length(min = 2, max = 1000))]
    pub name: String,
    #[validate(length(min = 2, max = 1000))]
    pub sub_district: String,
    #[validate(length(min = 2, max = 1000))]
    pub city: String,
    #[validate(length(min = 2, max = 1000))]
    pub province: String,
    pub latitude: String,
    pub longitude: String,
}

use crate::{
    event_handler::FCM,
    push_notif_handler::{FCMHandler, FCMPayloadData},
    types::NotifKind,
    util,
};

use std::thread;

#[derive(Deserialize, Validate)]
pub struct TestPushNotifQuery {
    pub loc: String,
    pub loc_kind: i16,
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Pandemia", "private", base = "/pandemia/v1")]
impl PrivateApi {
    /// Test push notif functionality, only for internal testing purposes.
    #[api_endpoint(path = "/test/push_notif", auth = "none", mutable)]
    pub fn test_push_notif(query: TestPushNotifQuery) -> ApiResult<()> {
        let conn = state.db();
        let _ = thread::spawn(move || {
            if let Err(e) = FCM.push(
                "fcm",
                &FCMPayloadData {
                    receiver_loc: &query.loc,
                    receiver_loc_kind: query.loc_kind.into(),
                    target_id: 0,
                    kind: NotifKind::NewCases,
                    title: "Test",
                    item: "",
                    message: "This is test message",
                    created: util::now(),
                    click_action: "FLUTTER_NOTIFICATION_CLICK",
                },
                &conn,
            ) {
                error!("Cannot test send push notif. {}", e);
            }
        });

        Ok(ApiResult::success(()))
    }
}
