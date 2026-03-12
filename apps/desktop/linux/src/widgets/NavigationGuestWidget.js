import GObject from 'gi://GObject';
import { NavigationBaseWidget } from './NavigationBaseWidget.js';

import '../pages/welcome/WelcomePage.js';
import '../pages/signin/SignInPage.js';
import '../pages/signup/SignUpPage.js';

export const NavigationGuestWidget = GObject.registerClass({
    GTypeName: 'YdxNavigationGuestWidget',
    Template: 'resource:///com/keygenqt/ydx/ui/widgets/NavigationGuestWidget.ui',
    InternalChildren: ['IdNavigation', 'IdToastOverlay'],
}, class extends NavigationBaseWidget {});
