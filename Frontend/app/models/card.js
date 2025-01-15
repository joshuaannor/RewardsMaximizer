import Model, { attr } from '@ember-data/model';

export default class CardModel extends Model {
  @attr('number') cardId;
  @attr('string') name;
  @attr('string') rtype;
  @attr('string') icon;
  @attr('string') color;
  @attr('string') benefits;
  @attr('string') category;
  @attr('number') rating;
  @attr('string') created;
  @attr('string') updated;
}
