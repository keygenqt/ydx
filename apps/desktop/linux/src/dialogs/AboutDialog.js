import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';

export const AboutDialog = GObject.registerClass({
	GTypeName: 'YdxAboutDialog',
    Template: 'resource:///com/keygenqt/ydx/ui/dialogs/AboutDialog.ui',
	InternalChildren: ['IdAboutDialog'],
}, class extends Gtk.Widget {
	present(window) {
		this._IdAboutDialog.present(window);
	}
});
