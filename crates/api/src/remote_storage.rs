use super::{api_fn, debug, virtual_struct};
use core::ptr;

virtual_struct! { RemoteStorage {
    fn file_write(&self, pch_file: *const u8, pv_data: *const (), cub_data: i32) -> bool,
    fn file_read(&self, pch_file: *const u8, pv_data: *const (), cub_data_to_read: i32) -> i32,
    fn file_write_async(&self, pch_file: *const u8, pv_data: *const u8, cub_data: u32) -> i32,
    fn file_read_async(&self, pch_file: *const u8, offset: u32, cub_to_read: u32) -> i32,
    fn file_read_async_complete(&self, read_call: i32, pv_buffer: *const u8, cub_to_read: u32) -> bool,
    fn file_forget(&self, pch_file: *const u8) -> bool,
    fn file_delete(&self, pch_file: *const u8) -> bool,
    fn file_share(&self, pch_file: *const u8) -> i32,
    fn set_sync_platforms(&self, pch_file: *const u8, remote_storage_platform: u8) -> bool,
    fn file_write_stream_open(&self, pch_file: *const u8) -> i32,
    fn file_write_stream_write_chunk(&self, handle: i32, pv_data: *const (), cub_data: i32) -> bool,
    fn file_write_stream_close(&self, handle: i32) -> bool,
    fn file_write_stream_cancel(&self, handle: i32) -> bool,
    fn file_exists(&self, pch_file: *const u8) -> bool,
    fn file_persisted(&self, pch_file: *const u8) -> bool,
    fn get_file_size(&self, pch_file: *const u8) -> i32,
    fn get_file_timestamp(&self, pch_file: *const u8) -> i64,
    fn get_sync_platforms(&self, pch_file: *const u8) -> u8,
    fn get_file_count(&self) -> i32,
    fn get_file_name_and_size(&self, file: i32, file_size_in_bytes: *mut i32) -> *const u8,
    fn get_quota(&self) -> i32,
    fn is_cloud_enabled_for_account(&self) -> bool,
    fn is_cloud_enabled_for_app(&self) -> bool,
    fn set_cloud_enabled_for_app(&self, enabled: bool) -> bool,
    fn ugc_download(&self, ugc_handle: i32, priority: u32) -> i32,
    fn get_ugc_download_progress(&self, ugc_handle: i32, bytes_downloaded: *mut i32, bytes_expected: *mut i32) -> bool,
    fn get_ugc_details(&self, ugc_handle: i32, app_id: *mut i32, name: *mut u8, file_size_in_bytes: *mut i32, steam_id_owner: *mut u64) -> bool,
    fn ugc_read(&self, ugc_handle: i32, data: *mut (), data_to_read: i32, offset: u32, action: u8) -> i32,
    fn get_cached_ugc_count(&self) -> i32,
    fn get_cached_ugc_handle(&self, cached_content: i32) -> i32,
    fn publish_workshop_file(&self, file: *const u8, preview_file: *const u8, comsumer_app_id: i32, title: *const u8, description: *const u8, visibility: u8, tags: *const (), workshop_file_kind: u8) -> i32,
    fn create_published_file_update_request(&self, unpublished_file_id: i32) -> i32,
    fn update_published_file_file(&self, handle: i32, file: *const u8) -> bool,
    fn update_published_file_preview_file(&self, handle: i32, preview_file: *const u8) -> bool,
    fn update_published_file_title(&self, handle: i32, title: *const u8) -> bool,
    fn update_published_file_description(&self, handle: i32, description: *const u8) -> bool,
    fn update_published_file_visibility(&self, handle: i32, visibility: u8) -> bool,
    fn update_published_file_tags(&self, handle: i32, tags: *const ()) -> bool,
    fn commit_published_file_update(&self, handle: i32) -> i32,
    fn get_published_file_details(&self, handle: i32, max_seconds_old: u32) -> i32,
    fn delete_published_file(&self, handle: i32) -> i32,
    fn enumerate_user_published_files(&self, start_index: u32) -> i32,
    fn subscribe_published_file(&self, handle: i32) -> i32,
    fn enumerate_user_subscribed_files(&self, handle: i32) -> i32,
    fn unsubscribe_published_file(&self, handle: i32) -> i32,
    fn get_published_item_vote_details(&self, handle: i32) -> i32,
    fn update_user_published_item_vote(&self, handle: i32) -> i32,
    fn get_user_published_item_vote_details(&self, handle: i32) -> i32,
    fn enumerate_user_shared_workshop_files(&self, steam_id: i32, start_index: u32, tags: *const (), excluded_tags: *const ()) -> i32,
    fn publish_video(&self, video_provider: u8, video_account: *const u8, video_identifier: *const u8, preview_file: *const u8, app_id: i32, title: *const u8, description: *const u8, visibility: u8, tags: *const ()) -> i32,
    fn set_user_published_file_action(&self, id: i32, action: u8) -> i32,
    fn enumerate_published_files_by_user_action(&self, action: u8, start_index: u32) -> i32,
    fn enumerate_published_workshop_files(&self, action: u8, start_index: u32) -> i32,
    fn ugc_download_to_location(&self, handle: i32, location: *const u8, priority: u32) -> i32,
} }

impl RemoteStorage {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                file_write: SteamAPI_ISteamRemoteStorage_FileWrite,
                file_read: SteamAPI_ISteamRemoteStorage_FileRead,
                file_write_async: SteamAPI_ISteamRemoteStorage_FileWriteAsync,
                file_read_async: SteamAPI_ISteamRemoteStorage_FileReadAsync,
                file_read_async_complete: SteamAPI_ISteamRemoteStorage_FileReadAsyncComplete,
                file_forget: SteamAPI_ISteamRemoteStorage_FileForget,
                file_delete: SteamAPI_ISteamRemoteStorage_FileDelete,
                file_share: SteamAPI_ISteamRemoteStorage_FileShare,
                set_sync_platforms: SteamAPI_ISteamRemoteStorage_SetSyncPlatforms,
                file_write_stream_open: SteamAPI_ISteamRemoteStorage_FileWriteStreamOpen,
                file_write_stream_write_chunk:
                    SteamAPI_ISteamRemoteStorage_FileWriteStreamWriteChunk,
                file_write_stream_close: SteamAPI_ISteamRemoteStorage_FileWriteStreamClose,
                file_write_stream_cancel: SteamAPI_ISteamRemoteStorage_FileWriteStreamCancel,
                file_exists: SteamAPI_ISteamRemoteStorage_FileExists,
                file_persisted: SteamAPI_ISteamRemoteStorage_FilePersisted,
                get_file_size: SteamAPI_ISteamRemoteStorage_GetFileSize,
                get_file_timestamp: SteamAPI_ISteamRemoteStorage_GetFileTimestamp,
                get_sync_platforms: SteamAPI_ISteamRemoteStorage_GetSyncPlatforms,
                get_file_count: SteamAPI_ISteamRemoteStorage_GetFileCount,
                get_file_name_and_size: SteamAPI_ISteamRemoteStorage_GetFileNameAndSize,
                get_quota: SteamAPI_ISteamRemoteStorage_GetQuota,
                is_cloud_enabled_for_account: SteamAPI_ISteamRemoteStorage_IsCloudEnabledForAccount,
                is_cloud_enabled_for_app: SteamAPI_ISteamRemoteStorage_IsCloudEnabledForApp,
                set_cloud_enabled_for_app: SteamAPI_ISteamRemoteStorage_SetCloudEnabledForApp,
                ugc_download: SteamAPI_ISteamRemoteStorage_UGCDownload,
                get_ugc_download_progress: SteamAPI_ISteamRemoteStorage_GetUGCDownloadProgress,
                get_ugc_details: SteamAPI_ISteamRemoteStorage_GetUGCDetails,
                ugc_read: SteamAPI_ISteamRemoteStorage_UGCRead,
                get_cached_ugc_count: SteamAPI_ISteamRemoteStorage_GetCachedUGCCount,
                get_cached_ugc_handle: SteamAPI_ISteamRemoteStorage_GetCachedUGCHandle,
                publish_workshop_file: SteamAPI_ISteamRemoteStorage_PublishWorkshopFile,
                create_published_file_update_request:
                    SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest,
                update_published_file_file: SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile,
                update_published_file_preview_file:
                    SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile,
                update_published_file_title: SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle,
                update_published_file_description:
                    SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription,
                update_published_file_visibility:
                    SteamAPI_ISteamRemoteStorage_UpdatePublishedFileVisibility,
                update_published_file_tags: SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags,
                commit_published_file_update:
                    SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate,
                get_published_file_details: SteamAPI_ISteamRemoteStorage_GetPublishedFileDetails,
                delete_published_file: SteamAPI_ISteamRemoteStorage_DeletePublishedFile,
                enumerate_user_published_files:
                    SteamAPI_ISteamRemoteStorage_EnumerateUserPublishedFiles,
                subscribe_published_file: SteamAPI_ISteamRemoteStorage_SubscribePublishedFiles,
                enumerate_user_subscribed_files:
                    SteamAPI_ISteamRemoteStorage_EnumerateUserSubscribedFiles,
                unsubscribe_published_file: SteamAPI_ISteamRemoteStorage_UnsubscribePublishedFile,
                get_published_item_vote_details:
                    SteamAPI_ISteamRemoteStorage_GetPublishedItemVoteDetails,
                update_user_published_item_vote:
                    SteamAPI_ISteamRemoteStorage_UpdateUserPublishedItemVote,
                get_user_published_item_vote_details:
                    SteamAPI_ISteamRemoteStorage_GetUserPublishedItemVoteDetails,
                enumerate_user_shared_workshop_files:
                    SteamAPI_ISteamRemoteStorage_EnumerateUserSharedWorkshopFiles,
                publish_video: SteamAPI_ISteamRemoteStorage_PublishVideo,
                set_user_published_file_action:
                    SteamAPI_ISteamRemoteStorage_SetUserPublishedFileAction,
                enumerate_published_files_by_user_action:
                    SteamAPI_ISteamRemoteStorage_EnumeratePublishedFilesByUserAction,
                enumerate_published_workshop_files:
                    SteamAPI_ISteamRemoteStorage_EnumeratePublishedWorkshopFiles,
                ugc_download_to_location: SteamAPI_ISteamRemoteStorage_UGCDownloadToLocation,
            },
        }
    }
}

api_fn! { FileWrite(&RemoteStorage, pch_file: *const u8, pv_data: *const (), cub_data: i32) -> bool {
    debug!();

    false
} }

api_fn! { FileRead(&RemoteStorage, pch_file: *const u8, pv_data: *const (), cub_data_to_read: i32) -> i32 {
    debug!();

    0
} }

api_fn! { FileWriteAsync(&RemoteStorage, pchfile: *const u8, pvdata: *const u8, cubdata: u32) -> i32 {
    debug!();

    0
} }

api_fn! { FileReadAsync(&RemoteStorage, pchfile: *const u8, offset: u32, cubtoread: u32) -> i32 {
    debug!();

    0
} }

api_fn! { FileReadAsyncComplete(&RemoteStorage, readcall: i32, pvbuffer: *const u8, cubtoread: u32) -> bool {
    debug!();

    false
} }

api_fn! { FileForget(&RemoteStorage, pchfile: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { FileDelete(&RemoteStorage, pchfile: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { FileShare(&RemoteStorage, pchfile: *const u8) -> i32 {
    debug!();

    0
} }

api_fn! { SetSyncPlatforms(&RemoteStorage, pchfile: *const u8, remotestorageplatform: u8) -> bool {
    debug!();

    false
} }

api_fn! { FileWriteStreamOpen(&RemoteStorage, pchfile: *const u8) -> i32 {
    debug!();

    0
} }

api_fn! { FileWriteStreamWriteChunk(&RemoteStorage, handle: i32, pvdata: *const (), cubdata: i32) -> bool {
    debug!();

    false
} }

api_fn! { FileWriteStreamClose(&RemoteStorage, handle: i32) -> bool {
    debug!();

    false
} }

api_fn! { FileWriteStreamCancel(&RemoteStorage, handle: i32) -> bool {
    debug!();

    false
} }

api_fn! { FileExists(&RemoteStorage, pchfile: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { FilePersisted(&RemoteStorage, pchfile: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { GetFileSize(&RemoteStorage, pchfile: *const u8) -> i32 {
    debug!();

    0
} }

api_fn! { GetFileTimestamp(&RemoteStorage, pchfile: *const u8) -> i64 {
    debug!();

    0
} }

api_fn! { GetSyncPlatforms(&RemoteStorage, pchfile: *const u8) -> u8 {
    debug!();

    0
} }

api_fn! { GetFileCount(&RemoteStorage) -> i32 {
    debug!();

    0
} }

api_fn! { GetFileNameAndSize(&RemoteStorage, file: i32, filesizeinbytes: *mut i32) -> *const u8 {
    debug!();

    ptr::null()
} }

api_fn! { GetQuota(&RemoteStorage) -> i32 {
    debug!();

    0
} }

api_fn! { IsCloudEnabledForAccount(&RemoteStorage) -> bool {
    debug!();

    false
} }

api_fn! { IsCloudEnabledForApp(&RemoteStorage) -> bool {
    debug!();

    false
} }

api_fn! { SetCloudEnabledForApp(&RemoteStorage, enabled: bool) -> bool {
    debug!();

    false
} }

api_fn! { UGCDownload(&RemoteStorage, ugchandle: i32, priority: u32) -> i32 {
    debug!();

    0
} }

api_fn! { GetUGCDownloadProgress(&RemoteStorage, ugchandle: i32, bytesdownloaded: *mut i32, bytesexpected: *mut i32) -> bool {
    debug!();

    false
} }

api_fn! { GetUGCDetails(&RemoteStorage, ugchandle: i32, appid: *mut i32, name: *mut u8, filesizeinbytes: *mut i32, steamidowner: *mut u64) -> bool {
    debug!();

    false
} }

api_fn! { UGCRead(&RemoteStorage, ugchandle: i32, data: *mut (), datatoread: i32, offset: u32, action: u8) -> i32 {
    debug!();

    0
} }

api_fn! { GetCachedUGCCount(&RemoteStorage) -> i32 {
    debug!();

    0
} }

api_fn! { GetCachedUGCHandle(&RemoteStorage, cachedcontent: i32) -> i32 {
    debug!();

    0
} }

api_fn! { PublishWorkshopFile(&RemoteStorage, file: *const u8, previewfile: *const u8, comsumerappid: i32, title: *const u8, description: *const u8, visibility: u8, tags: *const (), workshopfilekind: u8) -> i32 {
    debug!();

    0
} }

api_fn! { CreatePublishedFileUpdateRequest(&RemoteStorage, unpublishedfileid: i32) -> i32 {
    debug!();

    0
} }

api_fn! { UpdatePublishedFileFile(&RemoteStorage, handle: i32, file: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { UpdatePublishedFilePreviewFile(&RemoteStorage, handle: i32, previewfile: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { UpdatePublishedFileTitle(&RemoteStorage, handle: i32, title: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { UpdatePublishedFileDescription(&RemoteStorage, handle: i32, description: *const u8) -> bool {
    debug!();

    false
} }

api_fn! { UpdatePublishedFileVisibility(&RemoteStorage, handle: i32, visibility: u8) -> bool {
    debug!();

    false
} }

api_fn! { UpdatePublishedFileTags(&RemoteStorage, handle: i32, tags: *const ()) -> bool {
    debug!();

    false
} }

api_fn! { CommitPublishedFileUpdate(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { GetPublishedFileDetails(&RemoteStorage, handle: i32, maxsecondsold: u32) -> i32 {
    debug!();

    0
} }

api_fn! { DeletePublishedFile(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { EnumerateUserPublishedFiles(&RemoteStorage, startindex: u32) -> i32 {
    debug!();

    0
} }

api_fn! { SubscribePublishedFiles(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { EnumerateUserSubscribedFiles(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { UnsubscribePublishedFile(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { GetPublishedItemVoteDetails(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { UpdateUserPublishedItemVote(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { GetUserPublishedItemVoteDetails(&RemoteStorage, handle: i32) -> i32 {
    debug!();

    0
} }

api_fn! { EnumerateUserSharedWorkshopFiles(&RemoteStorage, steamid: i32, startindex: u32, tags: *const (), excludedtags: *const ()) -> i32 {
    debug!();

    0
} }

api_fn! { PublishVideo(&RemoteStorage, videoprovider: u8, videoaccount: *const u8, videoidentifier: *const u8, previewfile: *const u8, appid: i32, title: *const u8, description: *const u8, visibility: u8, tags: *const ()) -> i32 {
    debug!();

    0
} }

api_fn! { SetUserPublishedFileAction(&RemoteStorage, id: i32, action: u8) -> i32 {
    debug!();

    0
} }

api_fn! { EnumeratePublishedFilesByUserAction(&RemoteStorage, action: u8, startindex: u32) -> i32 {
    debug!();

    0
} }

api_fn! { EnumeratePublishedWorkshopFiles(&RemoteStorage, action: u8, startindex: u32) -> i32 {
    debug!();

    0
} }

api_fn! { UGCDownloadToLocation(&RemoteStorage, handle: i32, location: *const u8, priority: u32) -> i32 {
    debug!();

    0
} }
