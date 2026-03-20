
using Microsoft.Extensions.DependencyInjection;
using Microsoft.UI.Xaml;
using YDX.Native;
using YDX.Services;
using YDX.Views.Pages.Home;
using YDX.Views.Pages.SignIn;
using YDX.Views.Pages.SignUp;
using YDX.Views.Pages.Splash;
using YDX.Views.Pages.Welcome;
using YDX.Views.Windows;
using static YDX.Utils.AppConstants;

namespace YDX
{
    public partial class App : Application
    {
        public ServiceProvider Services { get; private set; } = null!;

        public static new App Current => (App)Application.Current;

        public static T GetService<T>() where T : class => (Current as App)!.Services.GetRequiredService<T>();

        public App()
        {
            Services = ConfigureServices();
            InitializeComponent();
        }

        private static ServiceProvider ConfigureServices()
        {
            var services = new ServiceCollection();

            services.AddSingleton<IDialogService, DialogService>();
            services.AddSingleton<ITitleService, TitleService>();
            services.AddSingleton<INavigationService, NavigationService>();
            services.AddSingleton<ISecureStorageService, SecureStorageService>();

            services.AddTransient<MainWindowViewModel>();
            services.AddTransient<SplashViewModel>();
            services.AddTransient<WelcomeViewModel>();
            services.AddTransient<SignInViewModel>();
            services.AddTransient<SignUpViewModel>();
            services.AddTransient<HomeViewModel>();

            return services.BuildServiceProvider();
        }

        protected override void OnLaunched(LaunchActivatedEventArgs args)
        {
            //Windows.Globalization.ApplicationLanguages.PrimaryLanguageOverride = "en-EN";

            // init http client
            NativeMethods.HttpClientInit(Api.BaseUrl, Api.Uuid);

            // create window
            var mainWindow = new MainWindow();

            // initialize dialog DI
            (Services.GetRequiredService<IDialogService>() as DialogService)?.Initialize(mainWindow);

            // show window
            mainWindow.Activate();
        }
    }
}
