using YDX.Base;
using YDX.Services;
using System.Windows.Input;

namespace YDX.Views.Pages.SignIn
{
    public class SignInViewModel : ViewModelBase
    {
        private readonly INavigationService _navigation;
        public ICommand GoBackCommand { get; }

        public SignInViewModel(INavigationService navigation)
        {
            _navigation = navigation;
            GoBackCommand = new RelayCommand(OnGoBack);
        }

        private void OnGoBack()
        {
            _navigation.GoBack();
        }
    }
}