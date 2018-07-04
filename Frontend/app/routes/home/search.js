import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model(params) {
        $("div").remove(".slim-profile-div");
        return $.getJSON('http://localhost:8000/search/' + params.term);
    },
    afterModel() {
    }
});