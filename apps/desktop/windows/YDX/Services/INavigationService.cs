using System.Windows.Controls;

namespace YDX.Services
{
    public interface INavigationService
    {
        void NavigateTo<T>() where T : Page;
        void GoBack();
        bool CanGoBack { get; }
    }
}