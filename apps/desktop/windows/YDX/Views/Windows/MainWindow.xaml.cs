using System.Windows;

namespace YDX.Views.Windows
{
    public partial class MainWindow : Window
    {
        public MainWindow(MainWindowViewModel viewModel)
        {
            InitializeComponent();
            DataContext = viewModel;
            Loaded += (s, e) => viewModel.Start();
        }
    }
}