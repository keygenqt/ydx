using System.Windows.Controls;
using Microsoft.Extensions.DependencyInjection;

namespace YDX.Services
{
    public class NavigationService : INavigationService
    {
        private readonly IServiceProvider _services;

        public NavigationService(IServiceProvider services)
        {
            _services = services;
        }

        private Frame GetMainFrame()
        {
            if (System.Windows.Application.Current.MainWindow is Views.Windows.MainWindow mainWindow)
            {
                return mainWindow.MainFrame;
            }
            throw new System.InvalidOperationException("MainWindow not ready");
        }

        public void NavigateTo<T>() where T : Page
        {
            var frame = GetMainFrame();
            var page = _services.GetRequiredService<T>();
            frame.Navigate(page);
        }

        public void GoBack()
        {
            var frame = GetMainFrame();
            if (frame.CanGoBack)
                frame.GoBack();
        }

        public bool CanGoBack
        {
            get
            {
                var frame = GetMainFrame();
                return frame.CanGoBack;
            }
        }
    }
}