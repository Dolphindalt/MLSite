import DS from 'ember-data';

export default DS.Model.extend({
    username: DS.attr('string'),
    admin: DS.attr('boolean'),
    date_created: DS.attr('string'),
    uuid: DS.attr('string'),
});
