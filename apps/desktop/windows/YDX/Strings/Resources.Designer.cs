using Windows.ApplicationModel.Resources;

namespace YDX.Strings
{
    public static class Resources
    {
        private static readonly ResourceLoader _loader = ResourceLoader.GetForViewIndependentUse();

        public static string Website => _loader.GetString("Website");
        public static string Close => _loader.GetString("Close");
        public static string Version => _loader.GetString("Version");
        public static string Yes => _loader.GetString("Yes");
        public static string No => _loader.GetString("No");
        public static string WelcomeTitle => _loader.GetString("WelcomeTitle");
        public static string WelcomeBack => _loader.GetString("WelcomeBack");
        public static string Email => _loader.GetString("Email");
        public static string Password => _loader.GetString("Password");
        public static string DontHaveAccount => _loader.GetString("DontHaveAccount");
        public static string CreateSpace => _loader.GetString("CreateSpace");
        public static string AlreadyHaveAccount => _loader.GetString("AlreadyHaveAccount");
        public static string PleaseWait => _loader.GetString("PleaseWait");
        public static string Feature1 => _loader.GetString("Feature1");
        public static string Feature2 => _loader.GetString("Feature2");
        public static string Feature3 => _loader.GetString("Feature3");
        public static string FetchData => _loader.GetString("FetchData");
        public static string FailedFetchData => _loader.GetString("FailedFetchData");
        public static string VerifyEmail => _loader.GetString("VerifyEmail");
        public static string VerifyPassword => _loader.GetString("VerifyPassword");
        public static string ServerEmailError => _loader.GetString("ServerEmailError");
        public static string ServerPasswordError => _loader.GetString("ServerPasswordError");
        public static string EmailExists => _loader.GetString("EmailExists");
        public static string UserNotFound => _loader.GetString("UserNotFound");
        public static string SignIn => _loader.GetString("SignIn");
        public static string SignUp => _loader.GetString("SignUp");
        public static string SplashBrewingCoffee => _loader.GetString("SplashBrewingCoffee");
        public static string SplashSteepingTea => _loader.GetString("SplashSteepingTea");
        public static string SplashTicklingDatabase => _loader.GetString("SplashTicklingDatabase");
        public static string SplashIgnitingReactor => _loader.GetString("SplashIgnitingReactor");
        public static string SplashInspectingTokens => _loader.GetString("SplashInspectingTokens");
        public static string SplashCastingSpells => _loader.GetString("SplashCastingSpells");
        public static string SplashPullingRabbit => _loader.GetString("SplashPullingRabbit");
        public static string SplashFeedingUnicorns => _loader.GetString("SplashFeedingUnicorns");
        public static string SplashAcceleratingNeutrons => _loader.GetString("SplashAcceleratingNeutrons");
        public static string SplashMixingReagents => _loader.GetString("SplashMixingReagents");
        public static string SplashWateringServer => _loader.GetString("SplashWateringServer");
        public static string About => _loader.GetString("About");
        public static string Logout => _loader.GetString("Logout");
        public static string Copyright => _loader.GetString("Copyright");
        public static string FailedParseUserData => _loader.GetString("FailedParseUserData");
        public static string LogoutConfirmationMessage => _loader.GetString("LogoutConfirmationMessage");
        public static string EmailErrorRequired => _loader.GetString("EmailErrorRequired");
        public static string EmailErrorInvalid => _loader.GetString("EmailErrorInvalid");
        public static string PasswordErrorRequired => _loader.GetString("PasswordErrorRequired");
        public static string PasswordErrorLength => _loader.GetString("PasswordErrorLength");
        public static string PasswordErrorComplexity => _loader.GetString("PasswordErrorComplexity");
    }
}