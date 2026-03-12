export const AppConstants = Object.freeze({
    Gtk: {
        schema: "com.keygenqt.ydx",
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
        AccessToken: 'access_token',
        RefreshToken: 'refresh_token',
    },
    SettingsKeys: {
        WindowWidth: 'window-width',
        WindowHeight: 'window-height',
    }
});
