using System.Windows;
using Microsoft.Extensions.DependencyInjection;
using YDX.Views.Windows;
using YDX.Services;
using YDX.Views.Pages.Welcome;
using YDX.Views.Pages.SignIn;
using YDX.Views.Pages.SignUp;
using YDX.Views.Pages.Home;
using YDX.Native;
using static YDX.Utils.AppConstants;

namespace YDX
{
    /// <summary>
    /// Application entry point with dependency injection setup
    /// </summary>
    public partial class App : Application
    {
        /// <summary>
        /// Gets the service provider for dependency injection
        /// </summary>
        public IServiceProvider Services { get; }

        /// <summary>
        /// Initializes a new instance of the App class
        /// </summary>
        public App()
        {
            Services = ConfigureServices();
        }

        /// <summary>
        /// Configures and builds the service collection
        /// </summary>
        /// <returns>Configured service provider</returns>
        private static IServiceProvider ConfigureServices()
        {
            var services = new ServiceCollection();

            // general window
            services.AddSingleton<MainWindowViewModel>();
            services.AddTransient<MainWindow>();

            // view modes
            services.AddTransient<WelcomeViewModel>();
            services.AddTransient<SignInViewModel>();
            services.AddTransient<SignUpViewModel>();
            services.AddTransient<HomeViewModel>();

            // pages
            services.AddTransient<WelcomePage>();
            services.AddTransient<SignInPage>();
            services.AddTransient<SignUpPage>();
            services.AddTransient<HomePage>();

            // navigation
            services.AddSingleton<INavigationService, NavigationService>();

            return services.BuildServiceProvider();
        }

        /// <summary>
        /// Called when the application starts
        /// </summary>
        protected override void OnStartup(StartupEventArgs e)
        {
            base.OnStartup(e);
            // init http client
            NativeMethods.HttpClientInit(Api.BaseUrl, Api.Uuid);
            // show main window
            var mainWindow = Services.GetRequiredService<MainWindow>();
            mainWindow.Show();
        }
    }
}