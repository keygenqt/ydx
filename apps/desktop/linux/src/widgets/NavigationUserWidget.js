import GObject from 'gi://GObject';
import { NavigationBaseWidget } from './NavigationBaseWidget.js';

import '../pages/home/HomePage.js';

export const NavigationUserWidget = GObject.registerClass({
    GTypeName: 'YdxNavigationUserWidget',
    Template: 'resource:///com/keygenqt/ydx/ui/widgets/NavigationUserWidget.ui',
    InternalChildren: ['IdNavigation', 'IdToastOverlay'],
}, class extends NavigationBaseWidget {});
