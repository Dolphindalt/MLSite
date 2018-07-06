import Route from '@ember/routing/route';

export default Route.extend({
    category: '',
    model(params) {
        this.set("category", params.category);
        return JSON.stringify({});
    },
    setupController(controller, model) {
        let new_model = JSON.parse(JSON.stringify({
            "category" : this.get('category'),
            model
        }));
        this._super(controller, new_model);
    }
});
