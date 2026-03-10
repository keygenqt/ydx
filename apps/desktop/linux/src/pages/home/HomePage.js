import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

export const HomePage = GObject.registerClass({
    GTypeName: 'YdxHomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/HomePage.ui',
    InternalChildren: [],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.HomePage;
    }

    vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
	}
});
