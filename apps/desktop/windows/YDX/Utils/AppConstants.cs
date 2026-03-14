namespace YDX.Utils
{
    public static class AppConstants
    {
        public static class Api
        {
            public const string BaseUrl = "http://localhost:3002";
            public const string Uuid = "uuid-windows";
        }

        public static class SecretKeys
        {
            public const string UserId = "user_id";
            public const string AccessToken = "access_token";
            public const string RefreshToken = "refresh_token";
        }

        public static class SettingsKeys
        {
            public const string WindowWidth = "window-width";
            public const string WindowHeight = "window-height";
            public const string Version = "data-version";
        }

        public const int DataVersion = 1;
    }
}