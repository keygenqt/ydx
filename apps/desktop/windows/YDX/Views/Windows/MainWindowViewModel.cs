using YDX.Services;
using YDX.Base;
using YDX.Views.Pages.Welcome;

namespace YDX.Views.Windows
{
    /// <summary>
    /// ViewModel for the main application window.
    /// Handles window-level properties and navigation state.
    /// </summary>
    public class MainWindowViewModel : ViewModelBase
    {
        private readonly INavigationService _navigation;

        public MainWindowViewModel(INavigationService navigation)
        {
            _navigation = navigation;
        }

        public void Start()
        {
            _navigation?.NavigateTo<WelcomePage>();
        }
    }
}