b2EdgeShape* EdgeShape_new() {
    return new b2EdgeShape();
}
void EdgeShape_drop(b2EdgeShape* self) {
    delete self;
}

b2Shape* EdgeShape_as_shape(b2EdgeShape* self) {
    return static_cast<b2Shape*>(self);
}
b2EdgeShape* Shape_as_edge_shape(b2Shape* self) {
    return static_cast<b2EdgeShape*>(self);
}

void EdgeShape_set_one_sided(b2EdgeShape* self,
                   const b2Vec2* v0, const b2Vec2* v1, const b2Vec2* v2, const b2Vec2* v3) {
    self->SetOneSided(*v0, *v1, *v2, *v3);
}

void EdgeShape_set_two_sided(b2EdgeShape* self,
                   const b2Vec2* v1, const b2Vec2* v2) {
    self->SetTwoSided(*v1, *v2);
}

b2Vec2 EdgeShape_get_v1(const b2EdgeShape* self) {
    return self->m_vertex1;
}

b2Vec2 EdgeShape_get_v2(const b2EdgeShape* self) {
    return self->m_vertex2;
}

bool EdgeShape_get_v0(const b2EdgeShape* self, b2Vec2* v0) {
    *v0 = self->m_vertex0;
    return self->m_oneSided;
}

bool EdgeShape_get_v3(const b2EdgeShape* self, b2Vec2* v3) {
    *v3 = self->m_vertex3;
    return self->m_oneSided;
}
