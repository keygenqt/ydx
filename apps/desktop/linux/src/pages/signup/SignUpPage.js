import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

export const SignUpPage = GObject.registerClass({
    GTypeName: 'YdxSignUpPage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/SignUpPage.ui',
    InternalChildren: [],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.SignUpPage;
    }

    vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
	}
});
