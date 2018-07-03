import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    category: "",
    thread_uuid: "",
    model(params) {
        this.set('category', params.category);
        this.set('thread_uuid', params.thread_uuid);
        return $.getJSON("http://127.0.0.1:8000/forums/thread/" + params.category + "/" + params.thread_uuid);
    },
    setupController(controller, model) {
        let new_model = JSON.parse(JSON.stringify({
            "category" : this.get('category'),
            "thread_uuid" : this.get('thread_uuid'),
            model
        }));
        this._super(controller, new_model);
    }
});
