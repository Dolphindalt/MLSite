import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model() {
        return $.getJSON("127.0.0.1:8000/home");
    }
});
