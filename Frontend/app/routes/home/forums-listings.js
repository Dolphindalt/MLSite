import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    category: "",
    page: "",
    model(params) {
        this.set('category', params.category);
        this.set('page', params.page);
        return $.getJSON("http://localhost:8000/forums/" + params.category + "/" + params.page);
    },
    setupController(controller, model) {
        let new_model = JSON.parse(JSON.stringify({
            "category" : this.get('category'),
            "page" : this.get('page'),
            model
        }));
        this._super(controller, new_model);
    }
});
