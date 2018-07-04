import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model(params) {
        if(!params.term)
            return null;
        
        return $.getJSON('http://localhost:8000/search/' + params.term);
    }
});