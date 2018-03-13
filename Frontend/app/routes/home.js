import Route from '@ember/routing/route';
import $ from 'jquery';

// This function here will construct a new
// object known as model which is a JSON
// structure of all NewsPost documents. Try
// printing them out so you know what is up.
export default Route.extend({
    model() {
        return $.getJSON("localhost:8000/home");
    }
});
