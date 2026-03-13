export const AppConstants = Object.freeze({
    Gtk: {
        schema: "com.keygenqt.ydx",
        dataVersion: 1
    },
    Api: {
        base_url: "http://0.0.0.0:3002",
        uuid: "uuid-linux", // @todo
    },
    Pages: {
        Home: 'tag-HomePage',
        SignIn: 'tag-SignInPage',
        SignUp: 'tag-SignUpPage',
        Welcome: 'tag-WelcomePage',
    },
    SecretKeys: {
        UserID: 'user_id',
        AccessToken: 'access_token',
        RefreshToken: 'refresh_token',
    },
    SettingsKeys: {
        Version: 'data-version',
        WindowWidth: 'window-width',
        WindowHeight: 'window-height',
    }
});
