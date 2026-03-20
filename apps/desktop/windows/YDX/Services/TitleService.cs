using System;
using System.Collections.Generic;
using YDX.Strings;
using YDX.Views.Pages.Home;
using YDX.Views.Pages.SignIn;
using YDX.Views.Pages.SignUp;

namespace YDX.Services;

public class TitleService : ITitleService
{
    private readonly Dictionary<Type, string> _titles = new()
{
    { typeof(SignInPage), Resources.SignIn },
    { typeof(SignUpPage), Resources.SignUp },
    { typeof(HomePage), "YDX" },
};

    private string _currentTitle = "";
    public string CurrentTitle => _currentTitle;

    public event EventHandler<string>? TitleChanged;

    public void SetTitle(Type pageType)
    {
        if (_titles.TryGetValue(pageType, out var title))
        {
            _currentTitle = title;
            TitleChanged?.Invoke(this, _currentTitle);
        }
        else
        {
            _currentTitle = "";
            TitleChanged?.Invoke(this, _currentTitle);
        }
    }
}
