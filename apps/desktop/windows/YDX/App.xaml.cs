using System;
using System.Windows;
using Microsoft.Extensions.DependencyInjection;
using YDX.ViewModels;
using YDX.ViewModels.Pages.Welcome;
using YDX.Views;

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

            services.AddSingleton<MainWindowViewModel>();
            services.AddTransient<WelcomeViewModel>();

            return services.BuildServiceProvider();
        }

        /// <summary>
        /// Called when the application starts
        /// </summary>
        protected override void OnStartup(StartupEventArgs e)
        {
            base.OnStartup(e);

            var viewModel = Services.GetRequiredService<MainWindowViewModel>();
            var mainWindow = new MainWindow
            {
                DataContext = viewModel
            };

            mainWindow.Show();
        }
    }
}