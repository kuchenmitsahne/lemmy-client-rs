use lemmy_api_common::{
    comment::{
        CreateComment, CreateCommentLike, CreateCommentReport, DeleteComment, DistinguishComment,
        EditComment, GetComment, GetComments, ListCommentLikes, ListCommentReports, RemoveComment,
        ResolveCommentReport, SaveComment,
    },
    community::{
        AddModToCommunity, BanFromCommunity, BlockCommunity, CreateCommunity, DeleteCommunity,
        EditCommunity, FollowCommunity, GetCommunity, HideCommunity, ListCommunities,
        RemoveCommunity, TransferCommunity,
    },
    custom_emoji::{CreateCustomEmoji, DeleteCustomEmoji, EditCustomEmoji},
    person::{
        AddAdmin, BanPerson, BlockPerson, ChangePassword, DeleteAccount, GetPersonDetails,
        GetPersonMentions, GetReplies, GetReportCount, Login, MarkCommentReplyAsRead,
        MarkPersonMentionAsRead, PasswordChangeAfterReset, PasswordReset, PersonMentionResponse,
        Register, SaveUserSettings, UpdateTotp, VerifyEmail,
    },
    post::{
        CreatePost, CreatePostLike, CreatePostReport, DeletePost, EditPost, FeaturePost, GetPost,
        GetPosts, GetSiteMetadata, ListPostLikes, ListPostReports, LockPost, MarkPostAsRead,
        RemovePost, ResolvePostReport,
    },
    private_message::{
        CreatePrivateMessage, CreatePrivateMessageReport, DeletePrivateMessage, EditPrivateMessage,
        GetPrivateMessages, ListPrivateMessageReports, MarkPrivateMessageAsRead,
    },
    site::{
        ApproveRegistrationApplication, BlockInstance, CreateSite, EditSite, FederatedInstances,
        GetModlog, InstanceWithFederationState, ListRegistrationApplications, PurgeComment,
        PurgeCommunity, PurgePerson, PurgePost, ResolveObject, Search,
    },
};
use serde::Serialize;

use crate::impl_marker_trait;

pub(crate) trait LemmyForm: Serialize {}

impl_marker_trait!(
    LemmyForm,
    [
        // Comments
        CreateComment,
        CreateCommentLike,
        CreateCommentReport,
        DeleteComment,
        DistinguishComment,
        EditComment,
        GetComment,
        GetComments,
        ListCommentLikes,
        ListCommentReports,
        RemoveComment,
        ResolveCommentReport,
        SaveComment,
        // Communities
        AddModToCommunity,
        BanFromCommunity,
        BlockCommunity,
        CreateCommunity,
        DeleteCommunity,
        EditCommunity,
        FollowCommunity,
        GetCommunity,
        HideCommunity,
        ListCommunities,
        RemoveCommunity,
        TransferCommunity,
        // Emojis
        CreateCustomEmoji,
        DeleteCustomEmoji,
        EditCustomEmoji,
        // Person
        AddAdmin,
        BanPerson,
        BlockPerson,
        ChangePassword,
        DeleteAccount,
        GetPersonDetails,
        GetPersonMentions,
        GetReplies,
        GetReportCount,
        Login,
        MarkCommentReplyAsRead,
        MarkPersonMentionAsRead,
        PasswordChangeAfterReset,
        PasswordReset,
        PersonMentionResponse,
        Register,
        SaveUserSettings,
        UpdateTotp,
        VerifyEmail,
        // Posts
        CreatePost,
        CreatePostLike,
        CreatePostReport,
        DeletePost,
        EditPost,
        FeaturePost,
        GetPost,
        GetPosts,
        GetSiteMetadata,
        ListPostLikes,
        ListPostReports,
        LockPost,
        MarkPostAsRead,
        RemovePost,
        ResolvePostReport,
        // Private Messages
        CreatePrivateMessage,
        CreatePrivateMessageReport,
        DeletePrivateMessage,
        EditPrivateMessage,
        GetPrivateMessages,
        ListPrivateMessageReports,
        MarkPrivateMessageAsRead,
        // Site
        ApproveRegistrationApplication,
        BlockInstance,
        CreateSite,
        EditSite,
        FederatedInstances,
        GetModlog,
        InstanceWithFederationState,
        ListRegistrationApplications,
        PurgeComment,
        PurgeCommunity,
        PurgePerson,
        PurgePost,
        ResolveObject,
        Search
    ]
);
