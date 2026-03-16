using System.Windows.Controls;

namespace YDX.Views.Pages.Welcome
{
    /// <summary>
    /// Welcome page with auth and loading states
    /// </summary>
    public partial class WelcomePage : Page
    {
        public WelcomePage(WelcomeViewModel viewModel)
        {
            InitializeComponent();
            DataContext = viewModel;
        }
    }
}