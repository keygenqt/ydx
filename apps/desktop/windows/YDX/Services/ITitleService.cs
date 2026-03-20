using Microsoft.UI.Xaml.Controls;
using System;

namespace YDX.Services;

public interface ITitleService
{
    string CurrentTitle { get; }
    event EventHandler<string>? TitleChanged;
    void SetTitle(Type pageType);
}
