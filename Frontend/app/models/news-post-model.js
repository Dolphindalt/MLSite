import DS from 'ember-data';

export default DS.Model.extend({
    title: DS.attr('string'),
    body: DS.attr('string'),
    author: DS.attr('string'),
    datetime: DS.attr('string'),
    uuid: DS.attr('string')
});
