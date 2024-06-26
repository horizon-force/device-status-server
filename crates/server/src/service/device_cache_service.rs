use crate::config::app_state::AppState;
use crate::model::device::Device;
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

pub(crate) struct DeviceCacheService {}

impl DeviceCacheService {
    pub(crate) async fn run(app_state: AppState) -> anyhow::Result<(), anyhow::Error> {
        Self::run_once(app_state.clone()).await?;
        Self::run_periodically(app_state, Duration::from_secs(60)).await?;
        Ok(())
    }

    async fn run_once(app_state: AppState) -> anyhow::Result<(), anyhow::Error> {
        // update device cache once
        // cron job to store all device data in-memory repeatedly over time
        let sched = JobScheduler::new().await?;
        sched
            .add(
                Job::new_one_shot_async(Duration::from_secs(0), move |_uuid, _l| {
                    let app_state = app_state.clone();

                    Box::pin(async move {
                        log::info!("Generating one-off instance of device cache...");
                        // Update device cache
                        Self::update_device_cache(app_state).await;
                    })
                })
                .expect("Unable to create cron job"),
            )
            .await?;
        sched.start().await?;
        Ok(())
    }

    async fn run_periodically(
        app_state: AppState,
        duration: Duration,
    ) -> anyhow::Result<(), anyhow::Error> {
        // cron job to store all device data in-memory repeatedly over time
        let sched = JobScheduler::new().await?;
        sched
            .add(
                Job::new_repeated_async(duration, move |uuid, mut l| {
                    let app_state = app_state.clone();

                    Box::pin(async move {
                        // Update device cache
                        Self::update_device_cache(app_state).await;

                        // Query the next execution time for this job
                        let next_tick = l.next_tick_for_job(uuid).await;
                        match next_tick {
                            Ok(Some(ts)) => log::info!("Next time for cache refresh is {:?}", ts),
                            _ => log::info!("Could not get next tick for cache refresh job"),
                        }
                    })
                })
                .expect("Unable to create cron job"),
            )
            .await?;
        sched.start().await?;
        Ok(())
    }

    async fn update_device_cache(app_state: AppState) {
        let now: DateTime<Utc> = Utc::now();
        let devices: Vec<Device> = app_state
            .redis_service
            .get_all()
            .await
            .expect("Unable to get all devices");
        let device_cache = app_state
            .device_cache
            .upgrade()
            .expect("Device cache is no longer available");
        let mut write_guard = device_cache.write().await;
        let cache_size = devices.len();
        for device in devices {
            write_guard.insert(device.id.clone(), device);
        }
        let update_time = Utc::now() - now;
        log::info!(
            "Device cache size is {:?} and took {}ms to generate",
            cache_size,
            update_time.num_milliseconds()
        );
    }
}
