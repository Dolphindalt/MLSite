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
        let my_page = Number(this.get('page'));
        let prev = my_page > 1 ? my_page - 1 : 0;
        let next = (model.len >= my_page + 1) ? my_page + 1 : 0;
        let new_model = JSON.parse(JSON.stringify({
            "category" : this.get('category'),
            "page" : this.get('page'),
            "prev" : prev,
            "next" : next,
            model
        }));
        this._super(controller, new_model);
    }
});
