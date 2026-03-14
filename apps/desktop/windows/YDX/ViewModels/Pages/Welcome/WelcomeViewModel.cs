using System.Windows.Input;
using YDX.Commands;

namespace YDX.ViewModels.Pages.Welcome
{
    /// <summary>
    /// ViewModel for Welcome page.
    /// Manages loading state and transition between loading and auth views.
    /// </summary>
    public class WelcomeViewModel : ViewModelBase
    {
        private bool _isLoading = true;

        /// <summary>
        /// Indicates whether the page is in loading state.
        /// </summary>
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

        /// <summary>
        /// True when auth controls should be visible.
        /// </summary>
        public bool ShowAuth => !IsLoading;

        /// <summary>
        /// True when loading controls should be visible.
        /// </summary>
        public bool ShowLoading => IsLoading;

        /// <summary>
        /// Command for Sign In button
        /// </summary>
        public ICommand SignInCommand { get; }

        /// <summary>
        /// Command for Sign Up button
        /// </summary>
        public ICommand SignUpCommand { get; }

        /// <summary>
        /// Initializes a new instance of the WelcomeViewModel class.
        /// Starts a 5-second loading simulation, then switches to auth view.
        /// </summary>
        public WelcomeViewModel()
        {
            // Initialize commands
            SignInCommand = new RelayCommand(OnSignIn);
            SignUpCommand = new RelayCommand(OnSignUp);

            // Simulate authentication check
            Task.Delay(5000).ContinueWith(_ =>
            {
                IsLoading = false;
            });
        }

        private void OnSignIn()
        {
            println("Sign In clicked");
        }

        private void OnSignUp()
        {
            println("Sign Up clicked");
        }
    }
}