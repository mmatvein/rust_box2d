i32 Joint_get_type(const b2Joint* self) {
    return self->GetType();
}
b2Body* Joint_get_body_a(b2Joint* self) {
    return self->GetBodyA();
}
b2Body* Joint_get_body_b(b2Joint* self) {
    return self->GetBodyB();
}
b2Vec2 Joint_get_anchor_a_virtual(const b2Joint* self) {
    return self->GetAnchorA();
}
b2Vec2 Joint_get_anchor_b_virtual(const b2Joint* self) {
    return self->GetAnchorB();
}
b2Vec2 Joint_get_reaction_force_virtual(const b2Joint* self,
                                        f32 inv_dt) {
    return self->GetReactionForce(inv_dt);
}
f32 Joint_get_reaction_torque_virtual(const b2Joint* self,
                                      f32 inv_dt) {
    return self->GetReactionTorque(inv_dt);
}
b2Joint* Joint_get_next(b2Joint* self) {
    return self->GetNext();
}
const b2Joint* Joint_get_next_const(const b2Joint* self) {
    return self->GetNext();
}
void* Joint_get_user_data(b2Joint* self) {
    return reinterpret_cast<void*>(self->GetUserData().pointer);
}
void Joint_set_user_data(b2Joint* self, void* data) {
    auto userData = self->GetUserData();
    userData.pointer = reinterpret_cast<uintptr_t>(data);
}
bool Joint_is_enabled(const b2Joint* self) {
    return self->IsEnabled();
}
bool Joint_get_collide_connected(const b2Joint* self) {
    return self->GetCollideConnected();
}

void Joint_dump_virtual(b2Joint* self) {
    self->Dump();
}
void Joint_shift_origin_virtual(b2Joint* self, const b2Vec2* origin) {
    self->ShiftOrigin(*origin);
}
