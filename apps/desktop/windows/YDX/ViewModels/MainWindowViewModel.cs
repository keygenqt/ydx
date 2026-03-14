namespace YDX.ViewModels
{
    /// <summary>
    /// ViewModel for the main application window.
    /// Handles window-level properties and navigation state.
    /// </summary>
    public class MainWindowViewModel : ViewModelBase
    {
        private string _title = "YDX";

        /// <summary>
        /// Gets or sets the window title.
        /// </summary>
        public string Title
        {
            get => _title;
            set => SetProperty(ref _title, value);
        }
    }
}