b2CircleShape* CircleShape_new() {
    return new b2CircleShape();
}

void CircleShape_drop(b2CircleShape* self) {
    delete self;
}

b2Shape* CircleShape_as_shape(b2CircleShape* self) {
    return static_cast<b2Shape*>(self);
}

b2CircleShape* Shape_as_circle_shape(b2Shape* self) {
    return static_cast<b2CircleShape*>(self);
}

b2Vec2 CircleShape_get_pos(const b2CircleShape* self) {
    return self->m_p;
}

void CircleShape_set_pos(b2CircleShape* self, b2Vec2 pos) {
    self->m_p = pos;
}
