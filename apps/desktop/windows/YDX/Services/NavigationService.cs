using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;
using System;

namespace YDX.Services;

public class NavigationService(ITitleService _titleService) : INavigationService
{
    private Frame? _frame;
    private bool _changeRoot = false;

    public event EventHandler? CanGoBackChanged;

    public void Initialize(Frame frame)
    {
        _frame = frame;
        _frame.Navigated += OnNavigated;
    }

    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        if (e.Content is Page page)
        {
            _titleService.SetTitle(page.GetType());
        }
        CanGoBackChanged?.Invoke(this, EventArgs.Empty);
    }

    public void NavigateTo<T>() where T : Page
    {
        _frame?.Navigate(typeof(T));
    }

    public void Replace<T>() where T : Page
    {
        if (_frame == null) return;

        _frame.Navigate(typeof(T));

        if (_frame.BackStack.Count > 0)
        {
            _frame.BackStack.RemoveAt(_frame.BackStack.Count - 1);
        }
    }

    public void ReplaceRoot<T>(bool animate = true) where T : Page
    {
        if (_frame == null) return;

        _changeRoot = animate;
        _frame.Navigate(typeof(T));

        if (animate)
        {
            var timer = new DispatcherTimer
            {
                Interval = TimeSpan.FromMilliseconds(10)
            };
            timer.Tick += (s, e) =>
            {
                timer.Stop();
                _frame.BackStack.Clear();
                _changeRoot = false;
                CanGoBackChanged?.Invoke(this, EventArgs.Empty);
            };
            timer.Start();
        }
        else
        {
            _frame.BackStack.Clear();
            CanGoBackChanged?.Invoke(this, EventArgs.Empty);
        }
    }

    public void GoBack()
    {
        if (_frame?.CanGoBack == true)
            _frame.GoBack();
    }

    public bool CanGoBack()
    {
        if (_changeRoot) return false;
        return _frame?.CanGoBack ?? false;
    }
}
