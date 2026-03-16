using System.Windows.Input;
using YDX.Native;
using YDX.Services;
using YDX.Views.Pages.SignIn;
using YDX.Views.Pages.SignUp;
using YDX.Views.Pages.Home;
using YDX.Base;

namespace YDX.Views.Pages.Welcome
{
    public class WelcomeViewModel : ViewModelBase
    {
        private readonly INavigationService _navigation;
        private bool _isLoading = true;

        public bool IsLoading
        {
            get => _isLoading;
            set
            {
                if (SetProperty(ref _isLoading, value))
                {
                    OnPropertyChanged(nameof(ShowAuth));
                    OnPropertyChanged(nameof(ShowLoading));
                }
            }
        }

        public bool ShowAuth => !IsLoading;
        public bool ShowLoading => IsLoading;

        public ICommand SignInCommand { get; }
        public ICommand SignUpCommand { get; }

        public WelcomeViewModel(INavigationService navigation)
        {
            _navigation = navigation;

            SignInCommand = new RelayCommand(OnSignIn);
            SignUpCommand = new RelayCommand(OnSignUp);

            Task.Delay(1000).ContinueWith(_ => CheckAuthToken());
        }

        private void CheckAuthToken()
        {
            try
            {
                string token = "TODO"; // TODO: из SecretService

                if (!string.IsNullOrEmpty(token))
                {
                    string response = NativeMethods.RequestSignCheck(token);
                    if (response.Contains("\"code\":200"))
                    {
                        println("User is authenticated, navigating to home");
                        _navigation.NavigateTo<HomePage>();
                        return;
                    }
                }
            }
            finally
            {
                IsLoading = false;
            }
        }

        private void OnSignIn()
        {
            _navigation.NavigateTo<SignInPage>();
        }

        private void OnSignUp()
        {
            _navigation.NavigateTo<SignUpPage>();
        }
    }
}