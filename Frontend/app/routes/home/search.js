import Route from '@ember/routing/route';
import $ from 'jquery';
import { run } from '@ember/runloop';

export default Route.extend({
    model(params) {
        if(!params.term)
            return null;
        
        return $.getJSON('http://localhost:8000/search/' + params.term);
    }
});