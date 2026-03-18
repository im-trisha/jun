use nt_time::FileTime;
use nt_time::chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let filetime = FileTime::from_unix_time_millis(date.and_utc().timestamp_millis());
    let filetime = filetime.map_err(serde::ser::Error::custom)?;
    serializer.serialize_u64(filetime.to_raw())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let ticks = u64::deserialize(deserializer)?;
    let date_time: DateTime<Utc> = FileTime::from(ticks).into();
    Ok(date_time.naive_local())
}
