using System.Windows;
using System.Windows.Controls;

namespace YDX.Views.Pages.Welcome.Elements
{
    /// <summary>
    /// Loading control with rotating messages while waiting for auth check
    /// </summary>
    public partial class WelcomeLoading : UserControl
    {
        private readonly List<string> _messages = new()
        {
            "☕ Brewing coffee",
            "🍵 Steeping tea",
            "🗄️ Tickling the database",
            "🚀 Igniting the reactor",
            "🔐 Inspecting tokens",
            "🧙 Casting authorization spells",
            "🎩 Pulling rabbit out of hat",
            "🦄 Feeding unicorns",
            "⚡ Accelerating neutrons",
            "🧪 Mixing reagents",
            "🌱 Watering the server"
        };

        private readonly Random _random = new();
        private int _currentMessageIndex = -1;

        public WelcomeLoading()
        {
            InitializeComponent();
            Loaded += OnLoaded;
            Unloaded += OnUnloaded;
        }

        /// <summary>
        /// Handles control load event
        /// </summary>
        private void OnLoaded(object sender, RoutedEventArgs e)
        {
            ShowMessage();
            _ = RotateMessagesAsync();
        }

        /// <summary>
        /// Shows random message different from previous one
        /// </summary>
        private void ShowMessage()
        {
            if (_messages.Count == 0) return;

            int newIndex;
            do
            {
                newIndex = _random.Next(_messages.Count);
            }
            while (newIndex == _currentMessageIndex && _messages.Count > 1);

            _currentMessageIndex = newIndex;
            MessageText.Text = _messages[_currentMessageIndex];
        }

        /// <summary>
        /// Rotates messages every 3 seconds while control is active
        /// </summary>
        private async Task RotateMessagesAsync()
        {
            while (_currentMessageIndex != -1)
            {
                await Task.Delay(3000);
                if (_currentMessageIndex == -1) return;
                ShowMessage();
            }
        }

        /// <summary>
        /// Handles control unload event
        /// </summary>
        private void OnUnloaded(object sender, RoutedEventArgs e)
        {
            _currentMessageIndex = -1;
        }
    }
}