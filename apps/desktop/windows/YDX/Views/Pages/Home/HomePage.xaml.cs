using System.Windows.Controls;
using YDX.Views.Pages.Home;

namespace YDX.Views.Pages.Home
{
    public partial class HomePage : Page
    {
        public HomePage(HomeViewModel viewModel)
        {
            InitializeComponent();
            DataContext = viewModel;
        }
    }
}